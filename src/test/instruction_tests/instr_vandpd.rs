use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 84, 209], OperandSize::Dword)
}

#[test]
fn vandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 1757998395, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 84, 177, 59, 237, 200, 104], OperandSize::Dword)
}

#[test]
fn vandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 84, 207], OperandSize::Qword)
}

#[test]
fn vandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1236143239, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 84, 36, 85, 135, 12, 174, 73], OperandSize::Qword)
}

#[test]
fn vandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 84, 228], OperandSize::Dword)
}

#[test]
fn vandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 791739516, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 84, 28, 197, 124, 252, 48, 47], OperandSize::Dword)
}

#[test]
fn vandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 84, 234], OperandSize::Qword)
}

#[test]
fn vandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1498768400, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 84, 132, 178, 16, 100, 85, 89], OperandSize::Qword)
}

#[test]
fn vandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 142, 84, 213], OperandSize::Dword)
}

#[test]
fn vandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 84, 35], OperandSize::Dword)
}

#[test]
fn vandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 63084492, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 156, 84, 175, 204, 151, 194, 3], OperandSize::Dword)
}

#[test]
fn vandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 245, 133, 84, 204], OperandSize::Qword)
}

#[test]
fn vandpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 237, 131, 84, 60, 199], OperandSize::Qword)
}

#[test]
fn vandpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 385831576, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 165, 148, 84, 132, 207, 152, 82, 255, 22], OperandSize::Qword)
}

#[test]
fn vandpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 171, 84, 228], OperandSize::Dword)
}

#[test]
fn vandpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 84, 0], OperandSize::Dword)
}

#[test]
fn vandpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 191, 84, 41], OperandSize::Dword)
}

#[test]
fn vandpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 173, 171, 84, 223], OperandSize::Qword)
}

#[test]
fn vandpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1731428146, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 189, 163, 84, 20, 253, 50, 127, 51, 103], OperandSize::Qword)
}

#[test]
fn vandpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 189, 191, 84, 52, 176], OperandSize::Qword)
}

#[test]
fn vandpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 84, 218], OperandSize::Dword)
}

#[test]
fn vandpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1440865336, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 203, 84, 172, 250, 56, 220, 225, 85], OperandSize::Dword)
}

#[test]
fn vandpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 222, 84, 60, 131], OperandSize::Dword)
}

#[test]
fn vandpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 245, 205, 84, 238], OperandSize::Qword)
}

#[test]
fn vandpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 157, 198, 84, 52, 115], OperandSize::Qword)
}

#[test]
fn vandpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM13)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 149, 222, 84, 23], OperandSize::Qword)
}

