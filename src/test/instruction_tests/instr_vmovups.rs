use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 213], OperandSize::Dword)
}

#[test]
fn vmovups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1086617216, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 148, 216, 128, 118, 196, 64], OperandSize::Dword)
}

#[test]
fn vmovups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 218], OperandSize::Qword)
}

#[test]
fn vmovups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RSI, 45963781, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 166, 5, 90, 189, 2], OperandSize::Qword)
}

#[test]
fn vmovups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 204], OperandSize::Dword)
}

#[test]
fn vmovups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 12, 247], OperandSize::Dword)
}

#[test]
fn vmovups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 203], OperandSize::Qword)
}

#[test]
fn vmovups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 63], OperandSize::Qword)
}

#[test]
fn vmovups_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 16, 209], OperandSize::Dword)
}

#[test]
fn vmovups_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 2016965394, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 16, 175, 18, 115, 56, 120], OperandSize::Dword)
}

#[test]
fn vmovups_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 124, 139, 16, 229], OperandSize::Qword)
}

#[test]
fn vmovups_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM30)), operand2: Some(IndirectDisplaced(RAX, 1845168732, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 124, 137, 16, 176, 92, 10, 251, 109], OperandSize::Qword)
}

#[test]
fn vmovups_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 16, 228], OperandSize::Dword)
}

#[test]
fn vmovups_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 16, 19], OperandSize::Dword)
}

#[test]
fn vmovups_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 124, 175, 16, 255], OperandSize::Qword)
}

#[test]
fn vmovups_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 805713073, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 124, 169, 16, 52, 189, 177, 52, 6, 48], OperandSize::Qword)
}

#[test]
fn vmovups_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 16, 198], OperandSize::Dword)
}

#[test]
fn vmovups_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1724406680, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 16, 164, 209, 152, 91, 200, 102], OperandSize::Dword)
}

#[test]
fn vmovups_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 124, 207, 16, 220], OperandSize::Qword)
}

#[test]
fn vmovups_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1856665844, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 16, 36, 245, 244, 120, 170, 110], OperandSize::Qword)
}

#[test]
fn vmovups_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 235], OperandSize::Dword)
}

#[test]
fn vmovups_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(EDI, 1182225356, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 167, 204, 83, 119, 70], OperandSize::Dword)
}

#[test]
fn vmovups_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 250], OperandSize::Qword)
}

#[test]
fn vmovups_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1627197375, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 132, 75, 191, 15, 253, 96], OperandSize::Qword)
}

#[test]
fn vmovups_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 211], OperandSize::Dword)
}

#[test]
fn vmovups_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(ECX, Four, 755034506, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 44, 141, 138, 233, 0, 45], OperandSize::Dword)
}

#[test]
fn vmovups_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 247], OperandSize::Qword)
}

#[test]
fn vmovups_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 4, 185], OperandSize::Qword)
}

#[test]
fn vmovups_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 16, 222], OperandSize::Dword)
}

#[test]
fn vmovups_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 12, 243], OperandSize::Dword)
}

#[test]
fn vmovups_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 16, 217], OperandSize::Qword)
}

#[test]
fn vmovups_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RDI, 789182069, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 17, 159, 117, 246, 9, 47], OperandSize::Qword)
}

#[test]
fn vmovups_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 16, 219], OperandSize::Dword)
}

#[test]
fn vmovups_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 15], OperandSize::Dword)
}

#[test]
fn vmovups_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 170, 16, 227], OperandSize::Qword)
}

#[test]
fn vmovups_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RSI, 1898657006, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 142, 238, 52, 43, 113], OperandSize::Qword)
}

#[test]
fn vmovups_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 16, 208], OperandSize::Dword)
}

#[test]
fn vmovups_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(EBX, 107835502, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 147, 110, 112, 109, 6], OperandSize::Dword)
}

#[test]
fn vmovups_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 16, 231], OperandSize::Qword)
}

#[test]
fn vmovups_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RBX, 1070004185, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 147, 217, 247, 198, 63], OperandSize::Qword)
}

