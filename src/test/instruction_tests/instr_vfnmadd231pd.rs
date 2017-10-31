use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 188, 227], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 188, 10], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 188, 232], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 188, 26], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 188, 223], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1203827159, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 188, 140, 223, 215, 241, 192, 71], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 188, 197], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RBX, 1754746744, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 188, 187, 120, 79, 151, 104], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 188, 198], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 188, 54], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 205394644, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 153, 188, 148, 127, 212, 18, 62, 12], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 141, 142, 188, 225], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 237, 142, 188, 28, 243], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 229, 153, 188, 18], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 188, 248], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 1296895142, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 171, 188, 150, 166, 12, 77, 77], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 188, 40], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 133, 173, 188, 205], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 2116705780, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 165, 175, 188, 4, 181, 244, 93, 42, 126], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 181, 182, 188, 10], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 255, 188, 218], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 204, 188, 63], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 217, 188, 28, 136], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 213, 218, 188, 206], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1826786995, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 221, 194, 188, 20, 77, 179, 142, 226, 108], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 229, 215, 188, 52, 74], OperandSize::Qword)
}

