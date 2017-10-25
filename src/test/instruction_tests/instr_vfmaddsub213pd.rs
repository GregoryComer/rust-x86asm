use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 166, 215], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1441626914, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 166, 180, 127, 34, 123, 237, 85], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 166, 220], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 166, 47], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 166, 207], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 694818126, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 166, 162, 78, 21, 106, 41], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 166, 192], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RCX, 1522639547, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 166, 129, 187, 162, 193, 90], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 166, 195], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 139, 166, 28, 126], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 158, 166, 16], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 149, 129, 166, 212], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RSI, 1055837642, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 237, 135, 166, 166, 202, 205, 238, 62], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 197, 157, 166, 36, 74], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 166, 231], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2056871050, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 166, 44, 205, 138, 92, 153, 122], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1574340175, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 166, 180, 147, 79, 134, 214, 93], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 221, 172, 166, 215], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 401793434, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 173, 163, 166, 12, 85, 154, 225, 242, 23], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 690311607, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 189, 183, 166, 140, 218, 183, 81, 37, 41], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 157, 166, 192], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1442477043, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 206, 166, 132, 195, 243, 115, 250, 85], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 219, 166, 62], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 221, 218, 166, 210], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2119424735, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 237, 206, 166, 60, 125, 223, 218, 83, 126], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 353368666, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 189, 210, 166, 156, 215, 90, 250, 15, 21], OperandSize::Qword)
}

