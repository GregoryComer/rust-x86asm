use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpexpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 137, 206], OperandSize::Dword)
}

fn vpexpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 770376243, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 137, 158, 51, 2, 235, 45], OperandSize::Dword)
}

fn vpexpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 125, 138, 137, 200], OperandSize::Qword)
}

fn vpexpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 137, 28, 176], OperandSize::Qword)
}

fn vpexpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 137, 205], OperandSize::Dword)
}

fn vpexpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 137, 1], OperandSize::Dword)
}

fn vpexpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 125, 175, 137, 217], OperandSize::Qword)
}

fn vpexpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1227386599, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 137, 52, 189, 231, 110, 40, 73], OperandSize::Qword)
}

fn vpexpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 137, 234], OperandSize::Dword)
}

fn vpexpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1361676510, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 137, 132, 179, 222, 136, 41, 81], OperandSize::Dword)
}

fn vpexpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 125, 206, 137, 207], OperandSize::Qword)
}

fn vpexpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 137, 36, 81], OperandSize::Qword)
}

