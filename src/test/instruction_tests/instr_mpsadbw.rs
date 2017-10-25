use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 205, 90], OperandSize::Dword)
}

#[test]
fn mpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 28, 177, 34], OperandSize::Dword)
}

#[test]
fn mpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 217, 50], OperandSize::Qword)
}

#[test]
fn mpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDI, 1065965266, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 167, 210, 86, 137, 63, 29], OperandSize::Qword)
}

