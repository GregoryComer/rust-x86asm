use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 211], OperandSize::Dword)
}

fn paddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 376661422, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 188, 83, 174, 101, 115, 22], OperandSize::Dword)
}

fn paddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 239], OperandSize::Qword)
}

fn paddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 48], OperandSize::Qword)
}

fn paddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 224], OperandSize::Dword)
}

fn paddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 755479628, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 44, 85, 76, 180, 7, 45], OperandSize::Dword)
}

fn paddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 233], OperandSize::Qword)
}

fn paddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1095821085, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 164, 241, 29, 231, 80, 65], OperandSize::Qword)
}

