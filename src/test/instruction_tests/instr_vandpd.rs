use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 84, 195], OperandSize::Dword)
}

#[test]
fn vandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 413718671, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 84, 183, 143, 216, 168, 24], OperandSize::Dword)
}

#[test]
fn vandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 84, 252], OperandSize::Qword)
}

#[test]
fn vandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 467241723, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 84, 148, 195, 251, 138, 217, 27], OperandSize::Qword)
}

#[test]
fn vandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 84, 215], OperandSize::Dword)
}

#[test]
fn vandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 84, 20, 155], OperandSize::Dword)
}

#[test]
fn vandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 84, 204], OperandSize::Qword)
}

#[test]
fn vandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1152622555, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 84, 4, 213, 219, 159, 179, 68], OperandSize::Qword)
}

#[test]
fn vandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 143, 84, 193], OperandSize::Dword)
}

#[test]
fn vandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 140, 84, 12, 72], OperandSize::Dword)
}

#[test]
fn vandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 154, 84, 49], OperandSize::Dword)
}

#[test]
fn vandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 237, 131, 84, 197], OperandSize::Qword)
}

#[test]
fn vandpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 143, 84, 46], OperandSize::Qword)
}

#[test]
fn vandpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RDI, 217641260, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 221, 145, 84, 175, 44, 241, 248, 12], OperandSize::Qword)
}

#[test]
fn vandpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 172, 84, 232], OperandSize::Dword)
}

#[test]
fn vandpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 1425663318, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 169, 84, 155, 86, 229, 249, 84], OperandSize::Dword)
}

#[test]
fn vandpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 137483992, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 187, 84, 163, 216, 214, 49, 8], OperandSize::Dword)
}

#[test]
fn vandpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 205, 167, 84, 209], OperandSize::Qword)
}

#[test]
fn vandpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 749170990, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 189, 166, 84, 44, 117, 46, 113, 167, 44], OperandSize::Qword)
}

#[test]
fn vandpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 975044534, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 181, 181, 84, 180, 90, 182, 255, 29, 58], OperandSize::Qword)
}

#[test]
fn vandpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 84, 203], OperandSize::Dword)
}

#[test]
fn vandpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EBX, 1834104278, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 84, 163, 214, 53, 82, 109], OperandSize::Dword)
}

#[test]
fn vandpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDI, 1170255521, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 223, 84, 167, 161, 174, 192, 69], OperandSize::Dword)
}

#[test]
fn vandpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 213, 203, 84, 232], OperandSize::Qword)
}

#[test]
fn vandpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 205, 84, 35], OperandSize::Qword)
}

#[test]
fn vandpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 237, 219, 84, 38], OperandSize::Qword)
}

