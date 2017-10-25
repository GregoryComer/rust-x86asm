use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 231], OperandSize::Dword)
}

#[test]
fn psubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 989815647, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 28, 133, 95, 99, 255, 58], OperandSize::Dword)
}

#[test]
fn psubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 227], OperandSize::Qword)
}

#[test]
fn psubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RCX, 2092444980, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 185, 52, 45, 184, 124], OperandSize::Qword)
}

#[test]
fn psubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 237], OperandSize::Dword)
}

#[test]
fn psubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 20, 90], OperandSize::Dword)
}

#[test]
fn psubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 252], OperandSize::Qword)
}

#[test]
fn psubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 31], OperandSize::Qword)
}

