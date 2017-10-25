use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpacksswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 99, 252], OperandSize::Dword)
}

fn vpacksswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 2104004882, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 99, 132, 155, 18, 145, 104, 125], OperandSize::Dword)
}

fn vpacksswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 99, 233], OperandSize::Qword)
}

fn vpacksswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 2034343535, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 99, 136, 111, 158, 65, 121], OperandSize::Qword)
}

fn vpacksswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 99, 211], OperandSize::Dword)
}

fn vpacksswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1796662296, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 99, 20, 197, 24, 228, 22, 107], OperandSize::Dword)
}

fn vpacksswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 99, 196], OperandSize::Qword)
}

fn vpacksswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 99, 14], OperandSize::Qword)
}

fn vpacksswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 99, 244], OperandSize::Dword)
}

fn vpacksswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 99, 32], OperandSize::Dword)
}

fn vpacksswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 125, 141, 99, 226], OperandSize::Qword)
}

fn vpacksswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 69081260, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 45, 129, 99, 4, 149, 172, 24, 30, 4], OperandSize::Qword)
}

fn vpacksswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 99, 212], OperandSize::Dword)
}

fn vpacksswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1214634280, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 99, 44, 117, 40, 217, 101, 72], OperandSize::Dword)
}

fn vpacksswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 85, 161, 99, 239], OperandSize::Qword)
}

fn vpacksswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 13, 172, 99, 12, 240], OperandSize::Qword)
}

fn vpacksswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 201, 99, 254], OperandSize::Dword)
}

fn vpacksswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 99, 58], OperandSize::Dword)
}

fn vpacksswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 99, 232], OperandSize::Qword)
}

fn vpacksswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 45, 195, 99, 4, 176], OperandSize::Qword)
}

