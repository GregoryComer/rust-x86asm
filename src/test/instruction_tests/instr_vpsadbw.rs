use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 207], OperandSize::Dword)
}

#[test]
fn vpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 246, 20, 139], OperandSize::Dword)
}

#[test]
fn vpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 246, 197], OperandSize::Qword)
}

#[test]
fn vpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 246, 28, 159], OperandSize::Qword)
}

#[test]
fn vpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 246, 198], OperandSize::Dword)
}

#[test]
fn vpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 899699929, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 246, 131, 217, 84, 160, 53], OperandSize::Dword)
}

#[test]
fn vpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 246, 219], OperandSize::Qword)
}

#[test]
fn vpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 246, 52, 122], OperandSize::Qword)
}

#[test]
fn vpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 246, 193], OperandSize::Dword)
}

#[test]
fn vpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 108519558, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 246, 172, 247, 134, 224, 119, 6], OperandSize::Dword)
}

#[test]
fn vpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 33, 45, 0, 246, 236], OperandSize::Qword)
}

#[test]
fn vpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 29, 0, 246, 60, 147], OperandSize::Qword)
}

#[test]
fn vpsadbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 246, 232], OperandSize::Dword)
}

#[test]
fn vpsadbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 692111437, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 246, 156, 207, 77, 200, 64, 41], OperandSize::Dword)
}

#[test]
fn vpsadbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 193, 21, 246, 218], OperandSize::Qword)
}

#[test]
fn vpsadbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1742051643, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 5, 246, 148, 247, 59, 153, 213, 103], OperandSize::Qword)
}

#[test]
fn vpsadbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 246, 213], OperandSize::Dword)
}

#[test]
fn vpsadbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 171447693, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 93, 72, 246, 36, 197, 141, 21, 56, 10], OperandSize::Dword)
}

#[test]
fn vpsadbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 161, 125, 64, 246, 215], OperandSize::Qword)
}

#[test]
fn vpsadbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1887284198, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 93, 72, 246, 132, 254, 230, 171, 125, 112], OperandSize::Qword)
}

