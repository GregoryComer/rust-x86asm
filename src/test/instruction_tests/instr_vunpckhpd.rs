use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 21, 243], OperandSize::Dword)
}

#[test]
fn vunpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1489857578, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 21, 28, 221, 42, 108, 205, 88], OperandSize::Dword)
}

#[test]
fn vunpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 21, 218], OperandSize::Qword)
}

#[test]
fn vunpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 925612738, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 21, 130, 194, 186, 43, 55], OperandSize::Qword)
}

#[test]
fn vunpckhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 21, 193], OperandSize::Dword)
}

#[test]
fn vunpckhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 21, 30], OperandSize::Dword)
}

#[test]
fn vunpckhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 21, 205], OperandSize::Qword)
}

#[test]
fn vunpckhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 21, 44, 82], OperandSize::Qword)
}

#[test]
fn vunpckhpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 21, 194], OperandSize::Dword)
}

#[test]
fn vunpckhpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1054188481, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 139, 21, 180, 95, 193, 163, 213, 62], OperandSize::Dword)
}

#[test]
fn vunpckhpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 2178140, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 159, 21, 188, 73, 92, 60, 33, 0], OperandSize::Dword)
}

#[test]
fn vunpckhpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 149, 132, 21, 194], OperandSize::Qword)
}

#[test]
fn vunpckhpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 173, 137, 21, 63], OperandSize::Qword)
}

#[test]
fn vunpckhpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 2108755050, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 21, 188, 89, 106, 12, 177, 125], OperandSize::Qword)
}

#[test]
fn vunpckhpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 170, 21, 219], OperandSize::Dword)
}

#[test]
fn vunpckhpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 547288958, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 172, 21, 12, 205, 126, 247, 158, 32], OperandSize::Dword)
}

#[test]
fn vunpckhpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1188685345, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 190, 21, 132, 191, 33, 230, 217, 70], OperandSize::Dword)
}

#[test]
fn vunpckhpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 149, 169, 21, 229], OperandSize::Qword)
}

#[test]
fn vunpckhpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 197, 163, 21, 34], OperandSize::Qword)
}

#[test]
fn vunpckhpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 133, 185, 21, 4, 187], OperandSize::Qword)
}

#[test]
fn vunpckhpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 203, 21, 249], OperandSize::Dword)
}

#[test]
fn vunpckhpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1441627958, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 21, 172, 72, 54, 127, 237, 85], OperandSize::Dword)
}

#[test]
fn vunpckhpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 219, 21, 12, 194], OperandSize::Dword)
}

#[test]
fn vunpckhpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 141, 196, 21, 208], OperandSize::Qword)
}

#[test]
fn vunpckhpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 651065884, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 229, 206, 21, 172, 72, 28, 122, 206, 38], OperandSize::Qword)
}

#[test]
fn vunpckhpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1505001049, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 181, 211, 21, 36, 69, 89, 126, 180, 89], OperandSize::Qword)
}

