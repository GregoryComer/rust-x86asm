use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 240], OperandSize::Dword)
}

fn vpmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 60, 152], OperandSize::Dword)
}

fn vpmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 217], OperandSize::Qword)
}

fn vpmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 12, 64], OperandSize::Qword)
}

fn vpmovsxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 241], OperandSize::Dword)
}

fn vpmovsxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 28, 209], OperandSize::Dword)
}

fn vpmovsxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 251], OperandSize::Qword)
}

fn vpmovsxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1713776617, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 12, 117, 233, 39, 38, 102], OperandSize::Qword)
}

fn vpmovsxdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 37, 193], OperandSize::Dword)
}

fn vpmovsxdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 37, 12, 147], OperandSize::Dword)
}

fn vpmovsxdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 125, 140, 37, 206], OperandSize::Qword)
}

fn vpmovsxdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM20)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 140, 37, 35], OperandSize::Qword)
}

fn vpmovsxdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 37, 198], OperandSize::Dword)
}

fn vpmovsxdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 632693377, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 37, 20, 149, 129, 34, 182, 37], OperandSize::Dword)
}

fn vpmovsxdq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 37, 228], OperandSize::Qword)
}

fn vpmovsxdq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1294963973, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 169, 37, 156, 241, 5, 149, 47, 77], OperandSize::Qword)
}

fn vpmovsxdq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 37, 245], OperandSize::Dword)
}

fn vpmovsxdq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 584969337, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 37, 44, 189, 121, 236, 221, 34], OperandSize::Dword)
}

fn vpmovsxdq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 37, 249], OperandSize::Qword)
}

fn vpmovsxdq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM14)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 37, 48], OperandSize::Qword)
}

