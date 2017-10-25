use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn idiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Word)
}

fn idiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(DI, 12, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 125, 12], OperandSize::Word)
}

fn idiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Dword)
}

fn idiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1384714528, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 60, 213, 32, 17, 137, 82], OperandSize::Dword)
}

fn idiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Qword)
}

fn idiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 60, 193], OperandSize::Qword)
}

fn idiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 251], OperandSize::Qword)
}

fn idiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(RBX, 1234422927, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 187, 143, 204, 147, 73], OperandSize::Qword)
}

fn idiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 255], OperandSize::Word)
}

fn idiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 5348, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 184, 228, 20], OperandSize::Word)
}

fn idiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 250], OperandSize::Dword)
}

fn idiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1832822092, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 60, 125, 76, 165, 62, 109], OperandSize::Dword)
}

fn idiv_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 250], OperandSize::Qword)
}

fn idiv_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 60, 216], OperandSize::Qword)
}

fn idiv_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 249], OperandSize::Word)
}

fn idiv_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(BX, 195, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 191, 195, 0], OperandSize::Word)
}

fn idiv_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 253], OperandSize::Dword)
}

fn idiv_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(EDI, 194330304, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 191, 192, 62, 149, 11], OperandSize::Dword)
}

fn idiv_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 253], OperandSize::Qword)
}

fn idiv_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1632929058, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 188, 255, 34, 133, 84, 97], OperandSize::Qword)
}

fn idiv_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 254], OperandSize::Qword)
}

fn idiv_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 606859515, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 188, 202, 251, 240, 43, 36], OperandSize::Qword)
}

