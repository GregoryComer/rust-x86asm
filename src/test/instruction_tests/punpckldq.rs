use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 216], OperandSize::Dword)
}

fn punpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(EAX, 1486818243, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 136, 195, 11, 159, 88], OperandSize::Dword)
}

fn punpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 228], OperandSize::Qword)
}

fn punpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1390523630, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 156, 219, 238, 180, 225, 82], OperandSize::Qword)
}

fn punpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 210], OperandSize::Dword)
}

fn punpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 199023524, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 12, 221, 164, 219, 220, 11], OperandSize::Dword)
}

fn punpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 211], OperandSize::Qword)
}

fn punpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1625853312, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 140, 136, 128, 141, 232, 96], OperandSize::Qword)
}

