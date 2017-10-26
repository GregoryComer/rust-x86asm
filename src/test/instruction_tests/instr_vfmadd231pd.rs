use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 184, 234], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 184, 1], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 184, 248], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RCX, 1756627661, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 184, 137, 205, 2, 180, 104], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 184, 224], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 184, 8], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 184, 238], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RBX, 27084434, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 184, 131, 146, 70, 157, 1], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 184, 224], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 184, 12, 139], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 211085489, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 154, 184, 162, 177, 232, 148, 12], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 237, 139, 184, 221], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 18509423, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 189, 134, 184, 148, 143, 111, 110, 26, 1], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 197, 158, 184, 30], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 184, 208], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 2049057333, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 175, 184, 161, 53, 34, 34, 122], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ESI, 667699809, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 187, 184, 142, 97, 74, 204, 39], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 189, 161, 184, 228], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 729812729, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 157, 175, 184, 180, 243, 249, 14, 128, 43], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1170061668, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 157, 189, 184, 188, 211, 100, 185, 189, 69], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 154, 184, 206], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 184, 58], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ESI, 449032067, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 218, 184, 158, 131, 175, 195, 26], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 157, 249, 184, 225], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RCX, 55592550, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 205, 199, 184, 145, 102, 70, 80, 3], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 403265221, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 133, 209, 184, 4, 197, 197, 86, 9, 24], OperandSize::Qword)
}

