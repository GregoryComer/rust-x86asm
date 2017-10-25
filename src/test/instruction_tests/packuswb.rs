use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn packuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 195], OperandSize::Dword)
}

fn packuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1893757984, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 4, 133, 32, 116, 224, 112], OperandSize::Dword)
}

fn packuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 221], OperandSize::Qword)
}

fn packuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 31], OperandSize::Qword)
}

fn packuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 206], OperandSize::Dword)
}

fn packuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 47913028, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 44, 245, 68, 24, 219, 2], OperandSize::Dword)
}

fn packuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 211], OperandSize::Qword)
}

fn packuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 26], OperandSize::Qword)
}

