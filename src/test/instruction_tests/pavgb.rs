use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 224], OperandSize::Dword)
}

fn pavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 200895556, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 60, 213, 68, 108, 249, 11], OperandSize::Dword)
}

fn pavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 223], OperandSize::Qword)
}

fn pavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 2051644396, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 12, 213, 236, 155, 73, 122], OperandSize::Qword)
}

fn pavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 241], OperandSize::Dword)
}

fn pavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 60, 131], OperandSize::Dword)
}

fn pavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 208], OperandSize::Qword)
}

fn pavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1545823018, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 36, 213, 42, 99, 35, 92], OperandSize::Qword)
}

