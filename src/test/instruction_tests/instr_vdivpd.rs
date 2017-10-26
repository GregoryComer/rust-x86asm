use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 94, 226], OperandSize::Dword)
}

#[test]
fn vdivpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 94, 54], OperandSize::Dword)
}

#[test]
fn vdivpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 94, 246], OperandSize::Qword)
}

#[test]
fn vdivpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 94, 59], OperandSize::Qword)
}

#[test]
fn vdivpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 94, 246], OperandSize::Dword)
}

#[test]
fn vdivpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1152683332, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 94, 20, 205, 68, 141, 180, 68], OperandSize::Dword)
}

#[test]
fn vdivpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 94, 198], OperandSize::Qword)
}

#[test]
fn vdivpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 94, 60, 72], OperandSize::Qword)
}

#[test]
fn vdivpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 143, 94, 247], OperandSize::Dword)
}

#[test]
fn vdivpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 94, 20, 254], OperandSize::Dword)
}

#[test]
fn vdivpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1385404851, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 153, 94, 132, 115, 179, 153, 147, 82], OperandSize::Dword)
}

#[test]
fn vdivpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 221, 129, 94, 224], OperandSize::Qword)
}

#[test]
fn vdivpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1960127289, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 189, 135, 94, 4, 93, 57, 43, 213, 116], OperandSize::Qword)
}

#[test]
fn vdivpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 753965102, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 173, 147, 94, 20, 141, 46, 152, 240, 44], OperandSize::Qword)
}

#[test]
fn vdivpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 173, 94, 238], OperandSize::Dword)
}

#[test]
fn vdivpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 169, 94, 24], OperandSize::Dword)
}

#[test]
fn vdivpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1663119222, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 188, 94, 36, 197, 118, 47, 33, 99], OperandSize::Dword)
}

#[test]
fn vdivpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 165, 173, 94, 198], OperandSize::Qword)
}

#[test]
fn vdivpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 205, 163, 94, 22], OperandSize::Qword)
}

#[test]
fn vdivpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 913937400, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 173, 191, 94, 4, 189, 248, 147, 121, 54], OperandSize::Qword)
}

#[test]
fn vdivpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 153, 94, 203], OperandSize::Dword)
}

#[test]
fn vdivpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 94, 15], OperandSize::Dword)
}

#[test]
fn vdivpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1945247072, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 219, 94, 188, 240, 96, 29, 242, 115], OperandSize::Dword)
}

#[test]
fn vdivpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 246, 94, 238], OperandSize::Qword)
}

#[test]
fn vdivpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1171709073, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 149, 193, 94, 60, 189, 145, 220, 214, 69], OperandSize::Qword)
}

#[test]
fn vdivpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 220, 94, 44, 79], OperandSize::Qword)
}

