use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisttp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(Memory(1408, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 14, 128, 5], OperandSize::Word)
}

#[test]
fn fisttp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledDisplaced(EDX, Two, 368454452, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 12, 85, 52, 43, 246, 21], OperandSize::Dword)
}

#[test]
fn fisttp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1395223550, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 140, 144, 254, 107, 41, 83], OperandSize::Qword)
}

#[test]
fn fisttp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 9], OperandSize::Word)
}

#[test]
fn fisttp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 12, 112], OperandSize::Dword)
}

#[test]
fn fisttp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1984685128, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 140, 242, 72, 228, 75, 118], OperandSize::Qword)
}

#[test]
fn fisttp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(BX, 12758, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 143, 214, 49], OperandSize::Word)
}

#[test]
fn fisttp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(EDI, 257596332, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 143, 172, 155, 90, 15], OperandSize::Dword)
}

#[test]
fn fisttp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1656629886, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 140, 186, 126, 42, 190, 98], OperandSize::Qword)
}

