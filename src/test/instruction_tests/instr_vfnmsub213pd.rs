use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 174, 227], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 174, 60, 130], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 174, 192], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 174, 12, 250], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 174, 246], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 174, 7], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 174, 254], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RAX, 309749699, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 174, 144, 195, 103, 118, 18], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 174, 238], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 139, 174, 63], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 159, 174, 51], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 165, 143, 174, 250], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 1478029434, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 237, 140, 174, 153, 122, 240, 24, 88], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 221, 149, 174, 60, 200], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 174, 197], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 174, 49], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 189, 174, 44, 115], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 253, 163, 174, 232], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 221, 165, 174, 44, 215], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 173, 183, 174, 12, 241], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 155, 174, 206], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 1292418512, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 174, 155, 208, 189, 8, 77], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 221, 174, 28, 145], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 221, 212, 174, 204], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 170160503, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 198, 174, 44, 213, 119, 113, 36, 10], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 2092120077, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 149, 218, 174, 180, 200, 13, 56, 179, 124], OperandSize::Qword)
}

