use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 184, 243], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 362713128, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 184, 12, 221, 40, 144, 158, 21], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 184, 212], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 184, 30], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 184, 200], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 184, 12, 198], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 184, 200], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 674050919, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 184, 52, 253, 103, 51, 45, 40], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 184, 244], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1679449130, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 184, 164, 214, 42, 92, 26, 100], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 156, 184, 52, 94], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 245, 130, 184, 254], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 189, 129, 184, 28, 255], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 213, 155, 184, 52, 131], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 170, 184, 244], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1628286814, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 184, 188, 147, 94, 175, 13, 97], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 1578861997, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 189, 184, 160, 173, 133, 27, 94], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 245, 169, 184, 238], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2054676281, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 157, 167, 184, 20, 125, 57, 223, 119, 122], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1410125049, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 213, 181, 184, 140, 198, 249, 204, 12, 84], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 220, 184, 200], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 274623984, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 202, 184, 172, 67, 240, 109, 94, 16], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 223, 184, 25], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 181, 254, 184, 213], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RBX, 1956838736, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 189, 201, 184, 163, 80, 253, 162, 116], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RCX, 1491024966, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 189, 223, 184, 169, 70, 60, 223, 88], OperandSize::Qword)
}

