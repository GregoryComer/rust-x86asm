use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 233], OperandSize::Dword)
}

fn vucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 191336151, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 4, 117, 215, 142, 103, 11], OperandSize::Dword)
}

fn vucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 240], OperandSize::Qword)
}

fn vucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 54], OperandSize::Qword)
}

fn vucomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 46, 251], OperandSize::Dword)
}

fn vucomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 34], OperandSize::Dword)
}

fn vucomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 113, 124, 24, 46, 203], OperandSize::Qword)
}

fn vucomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1076279607, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 20, 253, 55, 185, 38, 64], OperandSize::Qword)
}

