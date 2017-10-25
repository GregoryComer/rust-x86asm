use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpxord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 239, 196], OperandSize::Dword)
}

fn vpxord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1891454368, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 239, 4, 141, 160, 77, 189, 112], OperandSize::Dword)
}

fn vpxord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1507855129, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 157, 239, 153, 25, 11, 224, 89], OperandSize::Dword)
}

fn vpxord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 69, 129, 239, 194], OperandSize::Qword)
}

fn vpxord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 239, 12, 121], OperandSize::Qword)
}

fn vpxord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 109, 151, 239, 36, 183], OperandSize::Qword)
}

fn vpxord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 239, 235], OperandSize::Dword)
}

fn vpxord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 265587740, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 239, 52, 133, 28, 140, 212, 15], OperandSize::Dword)
}

fn vpxord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 185, 239, 36, 206], OperandSize::Dword)
}

fn vpxord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 61, 161, 239, 253], OperandSize::Qword)
}

fn vpxord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 586955540, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 45, 171, 239, 172, 246, 20, 59, 252, 34], OperandSize::Qword)
}

fn vpxord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 93, 191, 239, 25], OperandSize::Qword)
}

fn vpxord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 239, 246], OperandSize::Dword)
}

fn vpxord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDI, 1134281883, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 239, 143, 155, 196, 155, 67], OperandSize::Dword)
}

fn vpxord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 222, 239, 51], OperandSize::Dword)
}

fn vpxord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 101, 201, 239, 193], OperandSize::Qword)
}

fn vpxord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2044166453, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 117, 206, 239, 12, 93, 53, 129, 215, 121], OperandSize::Qword)
}

fn vpxord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 109, 221, 239, 4, 198], OperandSize::Qword)
}

