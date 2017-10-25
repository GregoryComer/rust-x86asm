use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetmantpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 142, 38, 213, 100], OperandSize::Dword)
}

fn vgetmantpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 137, 38, 12, 150, 46], OperandSize::Dword)
}

fn vgetmantpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2129265706, Some(OperandSize::Qword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 156, 38, 44, 189, 42, 4, 234, 126, 72], OperandSize::Dword)
}

fn vgetmantpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM12)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 83, 253, 142, 38, 228, 37], OperandSize::Qword)
}

fn vgetmantpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 253, 137, 38, 20, 222, 4], OperandSize::Qword)
}

fn vgetmantpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 671555473, Some(OperandSize::Qword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 115, 253, 156, 38, 4, 133, 145, 31, 7, 40, 40], OperandSize::Qword)
}

fn vgetmantpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 38, 197, 89], OperandSize::Dword)
}

fn vgetmantpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EBX, 1087971664, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 38, 131, 80, 33, 217, 64, 81], OperandSize::Dword)
}

fn vgetmantpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 38, 39, 109], OperandSize::Dword)
}

fn vgetmantpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM31)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 131, 253, 170, 38, 199, 40], OperandSize::Qword)
}

fn vgetmantpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM28)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 253, 169, 38, 39, 26], OperandSize::Qword)
}

fn vgetmantpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 189, 38, 4, 65, 55], OperandSize::Qword)
}

fn vgetmantpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 157, 38, 249, 117], OperandSize::Dword)
}

fn vgetmantpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1143145340, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 38, 188, 71, 124, 3, 35, 68, 125], OperandSize::Dword)
}

fn vgetmantpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 49751816, Some(OperandSize::Qword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 218, 38, 140, 250, 8, 39, 247, 2, 41], OperandSize::Dword)
}

fn vgetmantpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 253, 154, 38, 216, 19], OperandSize::Qword)
}

fn vgetmantpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1055609539, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 253, 203, 38, 60, 93, 195, 82, 235, 62, 117], OperandSize::Qword)
}

fn vgetmantpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1976714050, Some(OperandSize::Qword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 222, 38, 188, 81, 66, 67, 210, 117, 64], OperandSize::Qword)
}

