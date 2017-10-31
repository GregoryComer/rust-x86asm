use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 195], OperandSize::Dword)
}

#[test]
fn vucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 1275426666, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 186, 106, 119, 5, 76], OperandSize::Dword)
}

#[test]
fn vucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 243], OperandSize::Qword)
}

#[test]
fn vucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 1208297812, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 184, 84, 41, 5, 72], OperandSize::Qword)
}

#[test]
fn vucomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 46, 198], OperandSize::Dword)
}

#[test]
fn vucomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 44, 75], OperandSize::Dword)
}

#[test]
fn vucomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 49, 124, 24, 46, 255], OperandSize::Qword)
}

#[test]
fn vucomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1224189827, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 36, 85, 131, 167, 247, 72], OperandSize::Qword)
}

