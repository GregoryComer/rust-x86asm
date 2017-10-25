use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 252], OperandSize::Dword)
}

fn movapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 10], OperandSize::Dword)
}

fn movapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 230], OperandSize::Qword)
}

fn movapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 698462154, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 180, 118, 202, 175, 161, 41], OperandSize::Qword)
}

fn movapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 197], OperandSize::Dword)
}

fn movapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 56], OperandSize::Dword)
}

fn movapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 207], OperandSize::Qword)
}

fn movapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 414155595, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 188, 118, 75, 131, 175, 24], OperandSize::Qword)
}

