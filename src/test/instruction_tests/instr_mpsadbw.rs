use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 211, 37], OperandSize::Dword)
}

#[test]
fn mpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 17, 85], OperandSize::Dword)
}

#[test]
fn mpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 213, 71], OperandSize::Qword)
}

#[test]
fn mpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 514092341, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 44, 149, 53, 109, 164, 30, 33], OperandSize::Qword)
}

