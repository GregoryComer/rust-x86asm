use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 229], OperandSize::Dword)
}

fn punpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 36, 127], OperandSize::Dword)
}

fn punpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 195], OperandSize::Qword)
}

fn punpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 100859372, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 20, 77, 236, 253, 2, 6], OperandSize::Qword)
}

fn punpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 195], OperandSize::Dword)
}

fn punpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 4, 128], OperandSize::Dword)
}

fn punpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 246], OperandSize::Qword)
}

fn punpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 44, 194], OperandSize::Qword)
}

