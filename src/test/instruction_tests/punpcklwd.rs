use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 205], OperandSize::Dword)
}

fn punpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1028913083, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 12, 93, 187, 247, 83, 61], OperandSize::Dword)
}

fn punpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 196], OperandSize::Qword)
}

fn punpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1271807526, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 180, 151, 38, 62, 206, 75], OperandSize::Qword)
}

fn punpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 248], OperandSize::Dword)
}

fn punpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1744773849, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 164, 146, 217, 34, 255, 103], OperandSize::Dword)
}

fn punpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 198], OperandSize::Qword)
}

fn punpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 52, 176], OperandSize::Qword)
}

