use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 151, 194], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1735933456, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 151, 4, 93, 16, 62, 120, 103], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 151, 251], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 864998592, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 151, 60, 221, 192, 212, 142, 51], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 151, 248], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 833644488, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 151, 156, 94, 200, 103, 176, 49], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 151, 212], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 871389936, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 151, 164, 198, 240, 90, 240, 51], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 151, 249], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1439246822, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 151, 52, 221, 230, 41, 201, 85], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 157, 151, 20, 138], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 173, 131, 151, 231], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 181, 131, 151, 50], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 213, 155, 151, 15], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 151, 252], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 968305176, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 170, 151, 36, 93, 24, 42, 183, 57], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 187, 151, 4, 211], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 149, 166, 151, 223], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1712888177, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 205, 165, 151, 12, 93, 113, 153, 24, 102], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 183, 151, 32], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 188, 151, 223], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 151, 3], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 641758497, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 221, 151, 156, 86, 33, 117, 64, 38], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 213, 150, 151, 198], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1259147110, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 181, 194, 151, 172, 80, 102, 15, 13, 75], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1997088606, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 149, 223, 151, 156, 182, 94, 39, 9, 119], OperandSize::Qword)
}

