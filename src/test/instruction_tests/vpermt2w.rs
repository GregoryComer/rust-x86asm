use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermt2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 125, 225], OperandSize::Dword)
}

fn vpermt2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 125, 12, 123], OperandSize::Dword)
}

fn vpermt2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 213, 134, 125, 245], OperandSize::Qword)
}

fn vpermt2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 181, 132, 125, 44, 185], OperandSize::Qword)
}

fn vpermt2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 125, 243], OperandSize::Dword)
}

fn vpermt2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 169, 125, 28, 131], OperandSize::Dword)
}

fn vpermt2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 181, 163, 125, 217], OperandSize::Qword)
}

fn vpermt2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RSI, 1776583778, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 133, 174, 125, 134, 98, 132, 228, 105], OperandSize::Qword)
}

fn vpermt2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 125, 225], OperandSize::Dword)
}

fn vpermt2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1767573410, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 206, 125, 180, 65, 162, 7, 91, 105], OperandSize::Dword)
}

fn vpermt2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 189, 205, 125, 192], OperandSize::Qword)
}

fn vpermt2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RCX, 1637415338, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 133, 202, 125, 145, 170, 249, 152, 97], OperandSize::Qword)
}

