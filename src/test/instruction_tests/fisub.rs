use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fisub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(SI, 7739, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 164, 59, 30], OperandSize::Word)
}

fn fisub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 32], OperandSize::Dword)
}

fn fisub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1808013844, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 36, 221, 20, 26, 196, 107], OperandSize::Qword)
}

fn fisub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 33], OperandSize::Word)
}

fn fisub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1059526679, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 164, 139, 23, 24, 39, 63], OperandSize::Dword)
}

fn fisub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(RDI, 1430712950, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 167, 118, 242, 70, 85], OperandSize::Qword)
}

