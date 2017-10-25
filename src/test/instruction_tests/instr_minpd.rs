use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 201], OperandSize::Dword)
}

#[test]
fn minpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 36, 67], OperandSize::Dword)
}

#[test]
fn minpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 254], OperandSize::Qword)
}

#[test]
fn minpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 797648934, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 36, 189, 38, 40, 139, 47], OperandSize::Qword)
}

