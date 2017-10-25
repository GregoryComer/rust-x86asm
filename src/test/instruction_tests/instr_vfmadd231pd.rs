use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 184, 239], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 2035085257, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 184, 4, 213, 201, 239, 76, 121], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 184, 254], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 184, 4, 94], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 184, 226], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 184, 15], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 184, 232], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 759922581, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 184, 36, 181, 149, 127, 75, 45], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 184, 243], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 459922915, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 184, 12, 85, 227, 221, 105, 27], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 184, 44, 217], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 229, 138, 184, 196], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 246189625, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 133, 131, 184, 60, 189, 57, 142, 172, 14], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 133, 156, 184, 18], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 184, 239], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 338096932, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 184, 188, 155, 36, 243, 38, 20], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 510604859, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 188, 184, 52, 157, 59, 54, 111, 30], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 133, 169, 184, 200], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1428985365, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 229, 174, 184, 172, 119, 21, 150, 44, 85], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1110070120, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 178, 184, 52, 149, 104, 83, 42, 66], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 191, 184, 206], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 24488737, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 203, 184, 148, 154, 33, 171, 117, 1], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EBX, 356736188, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 218, 184, 179, 188, 92, 67, 21], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 229, 211, 184, 193], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1368996199, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 189, 205, 184, 20, 157, 103, 57, 153, 81], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1388403416, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 221, 217, 184, 44, 213, 216, 90, 193, 82], OperandSize::Qword)
}

