use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Word)
}

fn setb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 0], OperandSize::Word)
}

fn setb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Dword)
}

fn setb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectDisplaced(EAX, 986448152, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 128, 24, 1, 204, 58], OperandSize::Dword)
}

fn setb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

fn setb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 3], OperandSize::Qword)
}

fn setb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

fn setb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETB, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1807397665, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 205, 33, 179, 186, 107], OperandSize::Qword)
}

