use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn palignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 232, 42], OperandSize::Dword)
}

#[test]
fn palignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 36, 94, 34], OperandSize::Dword)
}

#[test]
fn palignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 239, 57], OperandSize::Qword)
}

#[test]
fn palignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 55, 9], OperandSize::Qword)
}

#[test]
fn palignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 197, 65], OperandSize::Dword)
}

#[test]
fn palignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 1719986911, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 134, 223, 234, 132, 102, 58], OperandSize::Dword)
}

#[test]
fn palignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 199, 15], OperandSize::Qword)
}

#[test]
fn palignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 1510246295, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 138, 151, 135, 4, 90, 102], OperandSize::Qword)
}

