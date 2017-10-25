use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrangeps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 138, 80, 237, 83], OperandSize::Dword)
}

fn vrangeps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1967089087, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 138, 80, 148, 254, 191, 101, 63, 117, 36], OperandSize::Dword)
}

fn vrangeps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 153, 80, 46, 45], OperandSize::Dword)
}

fn vrangeps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM13)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 195, 45, 129, 80, 253, 56], OperandSize::Qword)
}

fn vrangeps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 109, 133, 80, 20, 176, 72], OperandSize::Qword)
}

fn vrangeps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 13, 149, 80, 18, 16], OperandSize::Qword)
}

fn vrangeps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 174, 80, 214, 94], OperandSize::Dword)
}

fn vrangeps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 174, 80, 12, 83, 59], OperandSize::Dword)
}

fn vrangeps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDX, 1124087670, Some(OperandSize::Dword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 109, 187, 80, 130, 118, 55, 0, 67, 51], OperandSize::Dword)
}

fn vrangeps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM12)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 195, 13, 171, 80, 228, 27], OperandSize::Qword)
}

fn vrangeps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 61, 172, 80, 44, 126, 87], OperandSize::Qword)
}

fn vrangeps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RBX, 1065783589, Some(OperandSize::Dword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 61, 177, 80, 131, 37, 145, 134, 63, 46], OperandSize::Qword)
}

fn vrangeps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 69, 154, 80, 216, 28], OperandSize::Dword)
}

fn vrangeps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 201, 80, 60, 191, 61], OperandSize::Dword)
}

fn vrangeps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 101, 218, 80, 44, 251, 7], OperandSize::Dword)
}

fn vrangeps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM12)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 67, 117, 157, 80, 212, 97], OperandSize::Qword)
}

fn vrangeps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 61, 197, 80, 20, 183, 88], OperandSize::Qword)
}

fn vrangeps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 292279184, Some(OperandSize::Dword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 21, 210, 80, 148, 199, 144, 211, 107, 17, 63], OperandSize::Qword)
}

