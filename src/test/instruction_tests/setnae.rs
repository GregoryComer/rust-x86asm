use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Word)
}

fn setnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectDisplaced(BP, 170, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 134, 170, 0], OperandSize::Word)
}

fn setnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Dword)
}

fn setnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1232466094, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 150, 174, 240, 117, 73], OperandSize::Dword)
}

fn setnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

fn setnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 1], OperandSize::Qword)
}

fn setnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

fn setnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1245232812, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 85, 172, 190, 56, 74], OperandSize::Qword)
}

