use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM7)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 247, 10], OperandSize::Dword)
}

#[test]
fn pslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM3)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 243, 83], OperandSize::Qword)
}

#[test]
fn pslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 246, 127], OperandSize::Dword)
}

#[test]
fn pslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 246, 120], OperandSize::Qword)
}

#[test]
fn pslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 241], OperandSize::Dword)
}

#[test]
fn pslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 60, 154], OperandSize::Dword)
}

#[test]
fn pslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 223], OperandSize::Qword)
}

#[test]
fn pslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 969499237, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 172, 118, 101, 98, 201, 57], OperandSize::Qword)
}

#[test]
fn pslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 243], OperandSize::Dword)
}

#[test]
fn pslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 1070444427, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 174, 139, 175, 205, 63], OperandSize::Dword)
}

#[test]
fn pslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 247], OperandSize::Qword)
}

#[test]
fn pslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDI, 1575201545, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 183, 9, 171, 227, 93], OperandSize::Qword)
}

