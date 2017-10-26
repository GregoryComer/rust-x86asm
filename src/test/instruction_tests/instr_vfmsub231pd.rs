use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 186, 203], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 951596746, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 186, 135, 202, 54, 184, 56], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 186, 228], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 1212820127, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 186, 186, 159, 42, 74, 72], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 186, 243], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 186, 25], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 186, 198], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 186, 28, 190], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 141, 186, 244], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 186, 1], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 2063459655, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 186, 180, 199, 71, 229, 253, 122], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 133, 137, 186, 250], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RDX, 1692902789, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 181, 132, 186, 130, 133, 165, 231, 100], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 245, 151, 186, 36, 202], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 186, 255], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 186, 15], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 186, 44, 67], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 237, 172, 186, 252], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 582932308, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 157, 170, 186, 52, 117, 84, 215, 190, 34], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1940945551, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 141, 190, 186, 148, 223, 143, 122, 176, 115], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 253, 186, 238], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1203771059, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 186, 28, 245, 179, 22, 192, 71], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1730913865, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 217, 186, 52, 189, 73, 166, 43, 103], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 189, 223, 186, 223], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 173, 199, 186, 12, 182], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 189, 209, 186, 52, 223], OperandSize::Qword)
}

