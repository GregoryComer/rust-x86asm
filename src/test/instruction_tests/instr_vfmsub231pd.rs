use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 186, 227], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 186, 40], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 186, 228], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDI, 1496184275, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 186, 167, 211, 245, 45, 89], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 186, 210], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDX, 454466678, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 186, 146, 118, 156, 22, 27], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 186, 197], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 186, 28, 201], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 186, 221], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 710672645, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 186, 180, 193, 5, 1, 92, 42], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 159, 186, 44, 183], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 141, 131, 186, 232], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 157, 137, 186, 54], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RSI, 1579528816, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 189, 146, 186, 182, 112, 178, 37, 94], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 175, 186, 239], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 615996497, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 186, 148, 223, 81, 92, 183, 36], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1836069729, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 189, 186, 12, 77, 97, 51, 112, 109], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 173, 162, 186, 208], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDX, 1994488459, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 237, 171, 186, 130, 139, 122, 225, 118], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 733990425, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 187, 186, 148, 143, 25, 206, 191, 43], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 159, 186, 254], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 186, 60, 183], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 221, 186, 60, 201], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 229, 146, 186, 246], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1003622119, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 237, 198, 186, 44, 85, 231, 14, 210, 59], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 189, 211, 186, 26], OperandSize::Qword)
}

