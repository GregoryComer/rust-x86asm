use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lods_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(DI, 1886, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Word)
}

fn lods_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(EDX, 815963605, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Dword)
}

fn lods_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledDisplaced(RCX, Four, 818796767, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[172], OperandSize::Qword)
}

fn lods_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 255, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Word)
}

fn lods_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Dword)
}

fn lods_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1567175303, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Qword)
}

fn lods_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(BX, 32048, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 173], OperandSize::Word)
}

fn lods_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(ECX, 1609748058, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Dword)
}

fn lods_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(RCX, 327530808, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[173], OperandSize::Qword)
}

fn lods_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LODS, operand1: Some(IndirectDisplaced(RDX, 1797888848, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 173], OperandSize::Qword)
}

