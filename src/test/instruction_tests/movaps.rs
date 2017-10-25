use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 215], OperandSize::Dword)
}

fn movaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 858557809, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 132, 139, 113, 141, 44, 51], OperandSize::Dword)
}

fn movaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 235], OperandSize::Qword)
}

fn movaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RAX, 539825091, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 144, 195, 19, 45, 32], OperandSize::Qword)
}

fn movaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 218], OperandSize::Dword)
}

fn movaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 9], OperandSize::Dword)
}

fn movaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 197], OperandSize::Qword)
}

fn movaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1404778305, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 180, 150, 65, 55, 187, 83], OperandSize::Qword)
}

