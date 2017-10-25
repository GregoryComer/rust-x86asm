use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 249, 235], OperandSize::Dword)
}

fn vpsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1614741029, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 249, 164, 154, 37, 254, 62, 96], OperandSize::Dword)
}

fn vpsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 249, 229], OperandSize::Qword)
}

fn vpsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 249, 58], OperandSize::Qword)
}

fn vpsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 249, 199], OperandSize::Dword)
}

fn vpsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 249, 20, 254], OperandSize::Dword)
}

fn vpsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 249, 199], OperandSize::Qword)
}

fn vpsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 249, 48], OperandSize::Qword)
}

fn vpsubw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 249, 202], OperandSize::Dword)
}

fn vpsubw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 108029099, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 249, 142, 171, 100, 112, 6], OperandSize::Dword)
}

fn vpsubw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 29, 133, 249, 232], OperandSize::Qword)
}

fn vpsubw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 85, 142, 249, 52, 178], OperandSize::Qword)
}

fn vpsubw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 249, 228], OperandSize::Dword)
}

fn vpsubw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 249, 31], OperandSize::Dword)
}

fn vpsubw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 53, 169, 249, 237], OperandSize::Qword)
}

fn vpsubw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RBX, 545963400, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 167, 249, 139, 136, 189, 138, 32], OperandSize::Qword)
}

fn vpsubw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 202, 249, 228], OperandSize::Dword)
}

fn vpsubw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1815873674, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 249, 60, 149, 138, 8, 60, 108], OperandSize::Dword)
}

fn vpsubw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 77, 194, 249, 244], OperandSize::Qword)
}

fn vpsubw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1100906959, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 101, 204, 249, 188, 240, 207, 129, 158, 65], OperandSize::Qword)
}

