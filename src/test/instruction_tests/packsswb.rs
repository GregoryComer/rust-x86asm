use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn packsswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 239], OperandSize::Dword)
}

fn packsswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(ECX, 88509887, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 145, 191, 141, 70, 5], OperandSize::Dword)
}

fn packsswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 213], OperandSize::Qword)
}

fn packsswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 2042987130, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 172, 183, 122, 130, 197, 121], OperandSize::Qword)
}

fn packsswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 203], OperandSize::Dword)
}

fn packsswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 63], OperandSize::Dword)
}

fn packsswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 226], OperandSize::Qword)
}

fn packsswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 46], OperandSize::Qword)
}

