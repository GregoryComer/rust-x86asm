use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 198, 90], OperandSize::Dword)
}

#[test]
fn roundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 2115850412, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 188, 142, 172, 80, 29, 126, 100], OperandSize::Dword)
}

#[test]
fn roundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 218, 113], OperandSize::Qword)
}

#[test]
fn roundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 0, 102], OperandSize::Qword)
}

