use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Word)
}

fn setae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 2378, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 131, 74, 9], OperandSize::Word)
}

fn setae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Dword)
}

fn setae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1714564400, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 149, 48, 45, 50, 102], OperandSize::Dword)
}

fn setae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

fn setae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 854009485, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 152, 141, 38, 231, 50], OperandSize::Qword)
}

fn setae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

fn setae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectDisplaced(RDX, 117491388, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 130, 188, 198, 0, 7], OperandSize::Qword)
}

