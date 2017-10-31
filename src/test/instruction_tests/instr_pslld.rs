use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM3)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 243, 18], OperandSize::Dword)
}

#[test]
fn pslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM4)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 244, 75], OperandSize::Qword)
}

#[test]
fn pslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 244, 72], OperandSize::Dword)
}

#[test]
fn pslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 242, 99], OperandSize::Qword)
}

#[test]
fn pslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 194], OperandSize::Dword)
}

#[test]
fn pslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(EAX, 1850994124, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 160, 204, 237, 83, 110], OperandSize::Dword)
}

#[test]
fn pslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 215], OperandSize::Qword)
}

#[test]
fn pslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1661637349, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 148, 191, 229, 146, 10, 99], OperandSize::Qword)
}

#[test]
fn pslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 214], OperandSize::Dword)
}

#[test]
fn pslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 34], OperandSize::Dword)
}

#[test]
fn pslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 221], OperandSize::Qword)
}

#[test]
fn pslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 9], OperandSize::Qword)
}

