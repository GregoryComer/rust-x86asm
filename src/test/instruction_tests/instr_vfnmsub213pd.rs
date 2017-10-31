use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 174, 235], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 174, 52, 178], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 174, 226], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 443377653, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 174, 132, 215, 245, 103, 109, 26], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 174, 250], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDI, 2063960862, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 174, 175, 30, 139, 5, 123], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 174, 255], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 174, 62], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 174, 229], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 82629511, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 174, 140, 222, 135, 211, 236, 4], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 1831780439, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 155, 174, 169, 87, 192, 46, 109], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 149, 129, 174, 228], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 794054383, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 221, 142, 174, 172, 195, 239, 78, 84, 47], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 173, 150, 174, 60, 210], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 174, 198], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1538072078, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 174, 148, 72, 14, 30, 173, 91], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1015581372, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 174, 137, 188, 138, 136, 60], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 149, 167, 174, 203], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 328996102, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 189, 166, 174, 44, 141, 6, 21, 156, 19], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 141, 180, 174, 4, 81], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 156, 174, 237], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 174, 4, 254], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 119137306, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 174, 28, 205, 26, 228, 25, 7], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 213, 180, 174, 209], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 203, 174, 52, 130], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1235680294, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 213, 209, 174, 36, 77, 38, 252, 166, 73], OperandSize::Qword)
}

