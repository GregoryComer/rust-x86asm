use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 95, 229], OperandSize::Dword)
}

#[test]
fn vmaxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 95, 51], OperandSize::Dword)
}

#[test]
fn vmaxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 95, 200], OperandSize::Qword)
}

#[test]
fn vmaxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1905160219, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 95, 156, 74, 27, 112, 142, 113], OperandSize::Qword)
}

#[test]
fn vmaxps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 95, 229], OperandSize::Dword)
}

#[test]
fn vmaxps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1898737786, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 95, 172, 83, 122, 112, 44, 113], OperandSize::Dword)
}

#[test]
fn vmaxps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 95, 214], OperandSize::Qword)
}

#[test]
fn vmaxps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 95, 44, 198], OperandSize::Qword)
}

#[test]
fn vmaxps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 68, 140, 95, 213], OperandSize::Dword)
}

#[test]
fn vmaxps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 68, 140, 95, 52, 143], OperandSize::Dword)
}

#[test]
fn vmaxps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 108, 156, 95, 52, 121], OperandSize::Dword)
}

#[test]
fn vmaxps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 20, 129, 95, 202], OperandSize::Qword)
}

#[test]
fn vmaxps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 194998832, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 12, 132, 95, 148, 251, 48, 114, 159, 11], OperandSize::Qword)
}

#[test]
fn vmaxps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 36, 157, 95, 62], OperandSize::Qword)
}

#[test]
fn vmaxps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 172, 95, 197], OperandSize::Dword)
}

#[test]
fn vmaxps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 183995216, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 173, 95, 186, 80, 139, 247, 10], OperandSize::Dword)
}

#[test]
fn vmaxps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 187, 95, 57], OperandSize::Dword)
}

#[test]
fn vmaxps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 60, 163, 95, 232], OperandSize::Qword)
}

#[test]
fn vmaxps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 68, 161, 95, 60, 177], OperandSize::Qword)
}

#[test]
fn vmaxps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 179105240, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 28, 182, 95, 140, 219, 216, 237, 172, 10], OperandSize::Qword)
}

#[test]
fn vmaxps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 159, 95, 238], OperandSize::Dword)
}

#[test]
fn vmaxps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 21479686, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 205, 95, 44, 77, 6, 193, 71, 1], OperandSize::Dword)
}

#[test]
fn vmaxps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 217, 95, 36, 127], OperandSize::Dword)
}

#[test]
fn vmaxps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 52, 158, 95, 233], OperandSize::Qword)
}

#[test]
fn vmaxps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 92, 204, 95, 7], OperandSize::Qword)
}

#[test]
fn vmaxps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(RDX, 1326210063, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 92, 223, 95, 178, 15, 92, 12, 79], OperandSize::Qword)
}

