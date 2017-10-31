use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 172, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 629141688, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 172, 28, 77, 184, 240, 127, 37], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 172, 209], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 172, 1], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 172, 240], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 172, 49], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 172, 218], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1247110949, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 172, 36, 157, 37, 103, 85, 74], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 172, 248], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1115533179, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 172, 185, 123, 175, 125, 66], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 154, 172, 24], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 165, 129, 172, 193], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 2032580599, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 237, 138, 172, 132, 115, 247, 183, 38, 121], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 2063337426, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 173, 150, 172, 148, 200, 210, 7, 252, 122], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 172, 223], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1983014255, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 173, 172, 156, 190, 111, 101, 50, 118], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 514791518, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 187, 172, 129, 94, 24, 175, 30], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 229, 167, 172, 244], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1025530965, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 141, 162, 172, 188, 176, 85, 92, 32, 61], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 456472225, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 182, 172, 132, 217, 161, 54, 53, 27], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 252, 172, 199], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 602940438, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 201, 172, 183, 22, 36, 240, 35], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 219, 172, 52, 74], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 237, 177, 172, 208], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1970988533, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 245, 202, 172, 148, 211, 245, 229, 122, 117], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 215, 172, 20, 179], OperandSize::Qword)
}

