use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM6)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 246, 73], OperandSize::Dword)
}

fn psllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM7)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 247, 11], OperandSize::Qword)
}

fn psllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 241, 103], OperandSize::Dword)
}

fn psllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 245, 20], OperandSize::Qword)
}

fn psllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 223], OperandSize::Dword)
}

fn psllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 28, 243], OperandSize::Dword)
}

fn psllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 225], OperandSize::Qword)
}

fn psllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 15], OperandSize::Qword)
}

fn psllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 211], OperandSize::Dword)
}

fn psllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EBX, 1700753197, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 171, 45, 111, 95, 101], OperandSize::Dword)
}

fn psllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 193], OperandSize::Qword)
}

fn psllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 103442484, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 164, 64, 52, 104, 42, 6], OperandSize::Qword)
}

