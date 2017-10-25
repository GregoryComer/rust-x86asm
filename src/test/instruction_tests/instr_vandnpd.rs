use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 85, 238], OperandSize::Dword)
}

#[test]
fn vandnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1778101091, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 85, 180, 207, 99, 171, 251, 105], OperandSize::Dword)
}

#[test]
fn vandnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 85, 210], OperandSize::Qword)
}

#[test]
fn vandnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RSI, 86899307, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 85, 158, 107, 250, 45, 5], OperandSize::Qword)
}

#[test]
fn vandnpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 85, 242], OperandSize::Dword)
}

#[test]
fn vandnpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 248951468, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 85, 177, 172, 178, 214, 14], OperandSize::Dword)
}

#[test]
fn vandnpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 85, 236], OperandSize::Qword)
}

#[test]
fn vandnpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RAX, 1417119621, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 85, 144, 133, 135, 119, 84], OperandSize::Qword)
}

#[test]
fn vandnpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 138, 85, 234], OperandSize::Dword)
}

#[test]
fn vandnpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 324185804, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 85, 36, 157, 204, 174, 82, 19], OperandSize::Dword)
}

#[test]
fn vandnpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1107708597, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 154, 85, 52, 221, 181, 74, 6, 66], OperandSize::Dword)
}

#[test]
fn vandnpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 181, 137, 85, 196], OperandSize::Qword)
}

#[test]
fn vandnpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 189, 129, 85, 15], OperandSize::Qword)
}

#[test]
fn vandnpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 850874674, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 189, 148, 85, 148, 80, 50, 81, 183, 50], OperandSize::Qword)
}

#[test]
fn vandnpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 173, 85, 219], OperandSize::Dword)
}

#[test]
fn vandnpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 2112590777, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 170, 85, 12, 253, 185, 147, 235, 125], OperandSize::Dword)
}

#[test]
fn vandnpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 1901157994, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 188, 85, 131, 106, 94, 81, 113], OperandSize::Dword)
}

#[test]
fn vandnpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 157, 161, 85, 200], OperandSize::Qword)
}

#[test]
fn vandnpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RCX, 887246087, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 133, 174, 85, 153, 7, 77, 226, 52], OperandSize::Qword)
}

#[test]
fn vandnpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 237, 187, 85, 12, 78], OperandSize::Qword)
}

#[test]
fn vandnpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 203, 85, 247], OperandSize::Dword)
}

#[test]
fn vandnpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 201, 85, 32], OperandSize::Dword)
}

#[test]
fn vandnpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 219, 85, 30], OperandSize::Dword)
}

#[test]
fn vandnpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 165, 199, 85, 198], OperandSize::Qword)
}

#[test]
fn vandnpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 201, 85, 44, 198], OperandSize::Qword)
}

#[test]
fn vandnpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 2019652145, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 245, 222, 85, 60, 213, 49, 114, 97, 120], OperandSize::Qword)
}

