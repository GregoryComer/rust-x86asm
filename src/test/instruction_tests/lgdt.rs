use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectDisplaced(BX, 20572, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 151, 92, 80], OperandSize::Word)
}

fn lgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1091882692, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 148, 128, 196, 206, 20, 65], OperandSize::Dword)
}

fn lgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 991088956, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 148, 139, 60, 209, 18, 59], OperandSize::Qword)
}

