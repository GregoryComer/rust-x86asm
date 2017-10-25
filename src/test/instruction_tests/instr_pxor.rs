use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 215], OperandSize::Dword)
}

#[test]
fn pxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(ESI, 738844822, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 134, 150, 224, 9, 44], OperandSize::Dword)
}

#[test]
fn pxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 227], OperandSize::Qword)
}

#[test]
fn pxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RDX, 1154681471, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 130, 127, 10, 211, 68], OperandSize::Qword)
}

#[test]
fn pxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 221], OperandSize::Dword)
}

#[test]
fn pxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 16], OperandSize::Dword)
}

#[test]
fn pxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 254], OperandSize::Qword)
}

#[test]
fn pxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 52, 90], OperandSize::Qword)
}

