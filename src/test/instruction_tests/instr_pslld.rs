use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM4)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 244, 116], OperandSize::Dword)
}

#[test]
fn pslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM0)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 240, 77], OperandSize::Qword)
}

#[test]
fn pslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 244, 68], OperandSize::Dword)
}

#[test]
fn pslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 246, 104], OperandSize::Qword)
}

#[test]
fn pslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 242], OperandSize::Dword)
}

#[test]
fn pslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 18847835, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 44, 157, 91, 152, 31, 1], OperandSize::Dword)
}

#[test]
fn pslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 241], OperandSize::Qword)
}

#[test]
fn pslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1477099414, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 44, 181, 150, 191, 10, 88], OperandSize::Qword)
}

#[test]
fn pslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 252], OperandSize::Dword)
}

#[test]
fn pslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1539304736, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 12, 125, 32, 237, 191, 91], OperandSize::Dword)
}

#[test]
fn pslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 234], OperandSize::Qword)
}

#[test]
fn pslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RSI, 2102285789, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 190, 221, 85, 78, 125], OperandSize::Qword)
}

